use std;

import core::io;
import io::reader;
import io::reader_util;
import std::map;
import map::hashmap;

impl read_util for reader{  
  fn read_valid_char() -> char {
    let mut c;
    do {
      c = self.read_char();
    } while c == '\n' || c == ' ';
    ret c;
  }
  
  fn read_valid_line() -> str{
    let mut line;
    do {
      line = self.read_line();
    } while line == "";
    ret line;
  }

  fn read_uint() -> uint{
    let raw_uint = self.read_valid_line();
    ret option::get(uint::from_str(raw_uint));
  }
  fn read_int() -> int{
    ret option::get(int::from_str(self.read_line()));
  }
  fn read_uints() -> [uint]{
    fn to_uint(&&s: str) -> uint{
      ret option::get(uint::from_str(s));
    }
    let line = self.read_line();
    ret vec::map(str::split_char(line, ' '), to_uint);
  }
  fn read_ints() -> [int]{
    fn to_int(&&s: str) -> int{
      ret option::get(int::from_str(s));
    }
    let line = self.read_valid_line();
    ret vec::map(str::split_char(line, ' '), to_int);
  }
}

#[doc="input data type"]
type input = {
  mut index: uint,
  line: str
};

#[doc="output status"]
enum out_status {
  ready,
  start, 
  solved((uint, str))
}

#[doc="build input data from stdin"]
fn fetch(stdin: io::reader) -> input {
  ret {
    mut index: 0u, 
    line: stdin.read_valid_line()
  };
}

fn solve(in: input, mapping_table: @map::hashmap<str, str>) -> str {
  log(error, #fmt("idx.%u start solved", in.index));
  let s = vec::foldl("", str::chars(in.line)) {|acc, c|
    if !(*mapping_table).contains_key(str::from_char(c)) {
      fail(#fmt("not contain: %s", str::from_char(c)));
    }
    acc + (*mapping_table).get(str::from_char(c))
  };
  ret s;
}

fn mk_mapping_table() -> map::hashmap<str, str> {
  let mut mapping_table = map::str_hash::<str>();

  let training_data = [
    ("q", "z"), 
    ("a zoo", "y qee"), 
    ("ejp mysljylc kd kxveddknmc re jsicpdrysi", "our language is impossible to understand"), 
    ("rbcpc ypc rtcsra dkh wyfrepkym veddknkmkrkcd", "there are twenty six factorial possibilities"), 
    ("de kr kd eoya kw aej tysr re ujdr lkgc jv", "so it is okay if you want to just give up")];
  
  for training_data.each {|data|
    let (a, b) = data;
    assert(str::char_len(a) == str::char_len(b));
    vec::iter2(str::chars(a), str::chars(b)) {|c1, c2|
      log(error, (str::from_char(c1), str::from_char(c2)));
      mapping_table.insert(str::from_char(c1), str::from_char(c2));
    }
  }
  
  ret mapping_table;
}

#[test]
fn test_solve1() {
  let tbl = mk_mapping_table();
  assert(solve("ejp mysljylc kd kxveddknmc re jsicpdrysi", tbl) == "our language is impossible to understand");
  assert(solve("rbcpc ypc rtcsra dkh wyfrepkym veddknkmkrkcd", tbl) == "there are twenty six factorial possibilities");
  assert(solve("de kr kd eoya kw aej tysr re ujdr lkgc jv", tbl) == "so it is okay if you want to just give up");
}


fn main() {
  let stdin = io::stdin();
  let n = stdin.read_uint();
  let mut inputs : [input] = [];

  let tbl = mk_mapping_table();
  uint::range(0u, n) {|i| 
    let in = fetch(stdin);
    in.index = i;
    inputs += [in];
    let res = solve(in, @tbl);
    io::println(#fmt("Case #%u: %s", i+1u, res));
  }
}

