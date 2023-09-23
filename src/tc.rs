use std::{collections::HashMap, process::id, hash};

use crate::occParser;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
pub struct SemOcc
{
    pub ident: String,
    pub label: usize,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
pub enum Type
{
    Base {
        delta: Vec<SemOcc>,
        kappa: Vec<String>,
    },
    Abs {
        T1: Option<Box<Type>>,
        T2: Option<Box<Type>>,
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct TypeChecker
{
    pub gamma: HashMap<SemOcc, Type>,
    pub pi: Pi,
    pub occ: occParser::Occ,
	 pub assumption: Vec<(usize, Type)>,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
pub struct Pi 
{
	pub p: Vec<(usize, usize)>,
}

static mut next: usize = 0;

impl TypeChecker
{
	pub fn type_check(&mut self) -> Type
	{
		unsafe
		{
			next = 0;
		}

		return self.check();
	}

	fn check(&mut self) -> Type
	{
		match self.occ.expr.ExpType
		{
			occParser::Type::Const	=>	{ self.check_const() }
			occParser::Type::Var		=>	{ self.check_var() }
			occParser::Type::Fun		=>	{ self.check_func() }
			occParser::Type::App		=>	{ self.check_app() }
			occParser::Type::FApp	=>	{ self.check_fapp() }
			occParser::Type::Let		=>	{ self.check_let() }
			occParser::Type::Case	=> { self.check_case() }
			occParser::Type::Ref		=>	{ self.check_ref() }
			occParser::Type::RefW	=>	{ self.check_ref_write() }
			occParser::Type::RefR	=>	{ self.check_ref_read() }
         _ => { unreachable!("Either those doesn't work or are unimplemented!!!!"); }
		}
	}

	fn check_const(&mut self) -> Type
	{
		return Type::Base { delta: Vec::new(), kappa: Vec::new() };
	}

	fn check_var(&mut self) -> Type
	{
		let lookup: SemOcc = self.get_binding();

      let mut t: Type = match self.gamma.get(&lookup) { Some(val) => { (*val).clone() } None => { unreachable!() }};
		let mut delta: Vec<SemOcc> = Vec::new();
		delta.push(SemOcc { ident: self.occ.clone().expr.ident, label: self.occ.clone().label });

		return self.sq_union(t, Type::Base { delta, kappa: Vec::new() });
	}

	fn check_func(&mut self) -> Type
	{
		let _assumption = self.assumption.pop();
		let assumption = match _assumption { Some(assumption) => { assumption } None => { unreachable!("No assumption exists"); } };

		let key: SemOcc = SemOcc { ident: self.occ.clone().expr.ident, label: assumption.0 };
		self.gamma.insert(key.clone(), assumption.1.clone());

		let mut _tc2: TypeChecker = self.dive(self.occ.clone().expr.LHS);
		let t2: Type = _tc2.check();
		self.gamma.extend(_tc2.gamma);

		self.gamma.remove(&key);

		return Type::Abs { T1: Some(Box::new(assumption.1)), T2: Some(Box::new(t2)) };
	}

	fn check_app(&mut self) -> Type
	{
		let _tc2: TypeChecker = self.dive(self.occ.clone().expr.RHS);
		let t2: Type = _tc2.clone().check();
		self.gamma.extend(_tc2.gamma);
		self.assumption.push((_tc2.occ.label , t2.clone()));

		let mut _tc1: TypeChecker = self.dive(self.occ.clone().expr.LHS);
		let t1: Type = _tc1.check();
		self.gamma.extend(_tc1.gamma);

		match t1.clone()
		{
			Type::Abs { T1, T2 } =>
			{
				match T1 {
					Some(t) => 
					{ 
						if !self.base_equal(Box::into_inner(t), t2) { unreachable!("ERROR: function and parameter does not match") }
						match T2 { Some(t) => { return Box::into_inner(t); } None => { unreachable!(); } }
					}
					None => { unreachable!(); }
				}
			}
			_ => { unreachable!(); }
		}
	}

	fn check_fapp(&mut self) -> Type
	{
		let mut _tc1: TypeChecker = self.dive(self.occ.clone().expr.LHS);
		let t1: Type = _tc1.check();
		self.gamma.extend(_tc1.gamma);

		let mut _tc2: TypeChecker = self.dive(self.occ.clone().expr.RHS);
		let t2: Type = _tc2.check();
		self.gamma.extend(_tc2.gamma);

		return self.union(t1, t2);
	}

	fn check_let(&mut self) -> Type
	{
		let mut _tc1: TypeChecker = self.dive(self.occ.clone().expr.LHS);
		let t1: Type = _tc1.check();
		self.gamma.extend(_tc1.clone().gamma);

		let k = SemOcc { ident: self.occ.clone().expr.ident, label: self.occ.clone().label };
		self.gamma.insert(k.clone(), t1.clone());

		let mut _tc2: TypeChecker = self.dive(self.occ.clone().expr.RHS);
		let t2: Type = _tc2.check();
		self.gamma.extend(_tc2.clone().gamma);
		self.gamma.remove(&k);

		return t2;
	}

	fn check_case(&mut self) -> Type
	{
		let mut _tc1: TypeChecker = self.dive(self.occ.clone().expr.LHS);
		let t1: Type = _tc1.check();
		self.gamma.extend(_tc1.clone().gamma);

		let pats: Vec<occParser::Pat> = match self.occ.clone().expr.Pats { Some(p) => { p } None => { unreachable!("Error: the case constructor doesn not contain any patterns"); } };
		let occs: Vec<Box<occParser::Occ>> = match self.occ.clone().expr.Occs { Some(o) => { o } None => { unreachable!("Error: the case constructor doesn not contain any a set of occurrences"); } };

		let mut t: Type = t1.clone();
		for i in 0..pats.len()
		{
			let mut _x: String = "".to_string();
			let p = &pats[i];

			match (*p).clone()
			{ 
				occParser::Pat::Var(x) =>
				{
					_x = x.clone();
					self.gamma.insert(SemOcc { ident: _x, label: _tc1.clone().occ.label }, t1.clone());
				}
				_ => {}
			}
			let occ = &occs[i];
			let ti: Type = TypeChecker
			{
				gamma: self.gamma.clone(), 
				pi: self.pi.clone(),
				occ: Box::into_inner((*occ).clone()) ,
				assumption: self.assumption.clone(),
			}.check();

			t = self.union(t, ti);
		}

		return t;
	}

	fn check_ref(&mut self) -> Type
	{
		let mut _tc1: TypeChecker = self.dive(self.occ.clone().expr.LHS);
		let t1: Type = _tc1.check();
		self.gamma.extend(_tc1.gamma);

		let k: Vec<String>;
		unsafe
		{
			let id = "_nu".to_string() + &next.to_string();
			self.gamma.insert(SemOcc { ident: id.clone(), label: self.occ.clone().label }, t1);
			k = vec![id];
			next = next+1;
		}

		return Type::Base { delta: Vec::new(), kappa: k };
	}

	fn check_ref_write(&mut self) -> Type
	{
		let mut _tc1: TypeChecker = self.dive(self.occ.clone().expr.LHS);
		let t1: Type = _tc1.check();
		self.gamma.extend(_tc1.gamma);

		let mut _tc2: TypeChecker = self.dive(self.occ.clone().expr.RHS);
		let t2: Type = _tc2.check();
		self.gamma.extend(_tc2.gamma);

		let d1: Vec<SemOcc>;
		match t1.clone()
		{
			Type::Base { delta, kappa } =>
			{
				d1 = delta.clone();
				for location in kappa.clone()
				{
					self.gamma.insert(SemOcc { ident: location.clone(), label: self.occ.clone().label }, t2.clone());
				}
			}
			Type::Abs { T1, T2 } => { unreachable!(); }
		}

		return Type::Base { delta: d1, kappa: Vec::new() };
	}

	fn check_ref_read(&mut self) -> Type
	{
		let mut _tc: TypeChecker = self.dive(self.occ.clone().expr.LHS);
		let t1: Type = _tc.check();
		self.gamma.extend(_tc.gamma);

		let k: SemOcc = SemOcc { ident: self.occ.clone().expr.ident, label: self.occ.clone().label};

		let mut occurrences: Vec<SemOcc> =  Vec::new();
		let mut kappas: Vec<String> =  Vec::new();
		let mut nus: Vec<SemOcc> = Vec::new();

		match t1.clone()
		{
			Type::Base { delta, kappa } =>
			{
				for occ in delta.clone()
				{
					if !occurrences.contains(&occ.clone())
               { 
                   occurrences.push(occ);
               }
				}
				for location in kappa.clone()
				{
					if location.contains("_nu")
               {
                   nus.extend(self.get_all_bindings(location, self.occ.clone().label));
						 for nu in nus.clone()
						 {
							 if !occurrences.contains(&nu.clone())
                      {
                          occurrences.push(nu);
                      }
						 }
               }
				}
			}
			Type::Abs { T1, T2 } => { unreachable!(); }
		}

      for nu in nus
		{
			let lookup: Option<&Type> = self.gamma.get(&nu);
			match lookup.clone()
			{
				Some(t) =>
				{
					match t
					{
						Type::Base { delta, kappa } =>
						{
                      for occurence in delta.clone()
                      {
                          if !(occurrences.contains(&occurence))
                          {
                              occurrences.push(occurence)
                          }
                      }
                      for reference in kappa.clone()
                      {
                          if !(kappas.contains(&reference))
                          {
                              kappas.push(reference)
                          }
                      }
						}
						Type::Abs { T1, T2 } => { unreachable!("Error: mutable bindings cannot be abstractions"); }
					}
				}
				None => { println!("gamma {:#?}:", self.gamma); unreachable!("This should not have happend {:#?}:", nu.clone()); }
			}

      }

		let t2: Type = Type::Base { delta: occurrences, kappa: kappas };
		/*for key in occurrences.clone()
		{
			match self.gamma.get(&key)
			{
				Some(types) => { t2 = self.union(t2, types.clone()); }
				None => { unreachable!(); }
			}
		}*/

		return t2;
	}

	fn dive(&mut self, into: Option<Box<occParser::Occ>>) -> TypeChecker
	{
		TypeChecker
		{
			gamma: self.gamma.clone(), 
			pi: self.pi.clone(),
			occ: match into { Some(_occ) => { Box::into_inner(_occ) } None => { unreachable!() } },
			assumption: self.assumption.clone(),
		}
	}

	fn get_binding(&mut self) -> SemOcc
	{
		let ident: String = self.occ.clone().expr.ident;

		let mut g_bind: Option<usize> = None;
		let mut bind_occ: SemOcc = SemOcc { ident: ident.clone(), label: 0 };
		for key in self.gamma.keys()
		{
			if key.ident == ident
			{
				g_bind = Some(key.label.clone());
				bind_occ.label = key.label;
			}
		}
		if None == g_bind { unreachable!() }
		return bind_occ;
	}

	fn get_all_bindings(&mut self, location: String, point: usize) -> Vec<SemOcc> 
	{
		let pis: Vec<Pi> = self.pi.get_all_pi(point);
		let tot_pi: Pi = self.pi.construct_total();

		let mut res: Vec<SemOcc> = Vec::new();
		for _pi in pis
		{
			if self.gamma.keys().len() == 0 { continue; }

			let mut total: usize = 0;
			let mut found: bool = false;
			for key in self.gamma.keys()
			{
				if
					(key.clone().ident.contains(&location.to_string()) && tot_pi.p.contains(&(total, key.label))) || (key.clone().ident.contains(&location.to_string()) && key.clone().label == total) 
						{ total = key.label; found = true; }
			}
			if found { res.push(SemOcc { ident: location.clone(), label: total }); }
		}

		return res;
	}

	fn sq_union(&mut self, mut t1: Type, mut t2: Type) -> Type
	{
		match t1.clone()
		{
			Type::Base { delta, kappa} =>
			{
				return self.union(t1, t2);
			}
			Type::Abs { T1, T2 } =>
			{
				return match T2 { Some(t) => { self.sq_union(Box::into_inner(t), t2) } None => { unreachable!() } };
			}
		}
	}

	fn union(&mut self, mut t1: Type, mut t2: Type) -> Type
	{
		match t1
		{
			Type::Base { mut delta, mut kappa } =>
			{
				let mut d: Vec<SemOcc>;
				let mut k: Vec<String>;

				(d, k) = match t2 { Type::Base { delta, kappa } => {(delta,kappa)} _ => { unreachable!() } };

				for occ in d
				{
					if !delta.contains(&occ.clone()) { delta.push(occ); }
				}
				for loc in k
				{
					if !kappa.contains(&loc.clone()) { kappa.push(loc); }
				}

				return Type::Base { delta, kappa };
			}

			Type::Abs { T1, T2 } =>
			{
				let t1_1: Type = match T1 { Some(t) => { Box::into_inner(t) } None => { unreachable!() }};
				let t2_1: Type = match T2 { Some(t) => { Box::into_inner(t) } None => { unreachable!() }};

				let t1_2: Type;
				let t2_2: Type;

				(t1_2, t2_2) = match t2
				{
					Type::Abs { T1, T2 } =>
					{
						(
							match T1 { Some(t) => { Box::into_inner(t) } None => { unreachable!() }},
							match T2 { Some(t) => { Box::into_inner(t) } None => { unreachable!() }}
						)
					}
					_ => { unreachable!(); }
				};

				return Type::Abs { T1: Some(Box::new(self.union(t1_1, t2_1))), T2: Some(Box::new(self.union(t1_2, t2_2))) };
			}
		}
	}

	fn base_equal(&mut self, t1: Type, t2: Type) -> bool
	{
		let d1: Vec<SemOcc>;
		let k1: Vec<String>;
		(d1,k1) = match t1 { Type::Base { delta, kappa } => { (delta, kappa) } _ => { unreachable!() } };

		let d2: Vec<SemOcc>;
		let k2: Vec<String>;
		(d2,k2) = match t2 { Type::Base { delta, kappa } => { (delta, kappa) } _ => { unreachable!() } };

		if d1.eq(&d2) && k1.eq(&k2) { return true; }
		return false;
	}
}

impl Pi
{
	pub fn construct_from_hash(&mut self, map: HashMap<usize, usize>)
	{
		let mut vec: Vec<(usize, usize)> = Vec::new();
		for (key, val) in map
		{
			vec.push((key, val));
		}
		self.p = vec;
	}

	pub fn construct_total(&mut self) -> Pi
	{
		let mut condition: bool = true;
		let mut res: Pi = self.clone();

		while condition
		{
			let prev: Vec<(usize, usize)> = res.p.clone();

			for (key, val) in res.p.clone()
			{
				let vals: Vec<usize> = res.clone().find_values(val);
				for v in vals
				{
					if !res.p.contains(&(key.clone(), v.clone())) { res.p.push((key, v)); }
				}
			}

			if res.p == prev { return res; }
		}

		return res;
	}

	pub fn get_all_pi(&mut self, point: usize) -> Vec<Pi>
	{
		let keys: Vec<usize> = self.find_keys(point);
		let mut pi: Pi = self._find_keys(point);

		if keys.clone().len() == 0 { return vec![pi]; }
		else if keys.clone().len() == 1
		{
			let mut res: Vec<Pi> = self.get_all_pi(keys.clone()[0]);
			let mut _res: Vec<Pi> = Vec::new();
			for mut r in res.clone()
			{
				r.p.append(&mut pi.p);
				_res.append(&mut vec![r]);
			}
			return _res;
		}
		else
		{
			let mut res: Vec<Pi> = Vec::new();
			for key in keys.clone()
			{
				let mut temp: Vec<Pi> = self.get_all_pi(key.clone());
				let mut _temp: Vec<Pi> = Vec::new();
				for mut r in temp.clone()
				{
					r.p.push((key, point));
					_temp.append(&mut vec![r]);
				}
				res.append(&mut _temp);
			}
			return res;
		}
	}

	pub fn find_keys(&mut self, val: usize) -> Vec<usize>
	{
		let mut res: Vec<usize> = Vec::new();
		for (_key, _val) in self.p.clone()
		{
			if val == _val { res.push(_key); }
		}

		return res;
	}

	fn _find_keys(&mut self, val: usize) -> Pi
	{
		let mut res: Vec<(usize, usize)> = Vec::new();
		for (_key, _val) in self.p.clone()
		{
			if val == _val { res.push((_key, val)); }
		}

		return Pi { p: res };
	}

	pub fn find_values(&mut self, key: usize) -> Vec<usize>
	{
		let mut res: Vec<usize> = Vec::new();
		for (_key, _val) in self.p.clone()
		{
			if key == _key { res.push(_val); }
		}

		return res;
	}
}
