use clroxide::clr::Clr;

pub fn exec_ass(assembly: Vec<&str>) -> String {
    let mut clr = Clr::new(base64::decode(assembly[0]).unwrap(), vec![assembly[1].to_string()]).unwrap();
    let results = clr.run().unwrap();
    results
}