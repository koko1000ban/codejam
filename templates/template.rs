use std;

import core::io;
import io::reader;
import io::reader_util;

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
  w: uint, 
  h: uint, 
  board: [str]
};

#[doc="output status"]
enum out_status {
  ready,
  start, 
  solved((uint, str))
}

#[doc="build input data from stdin"]
fn fetch(stdin: io::reader) -> input {
  let h = stdin.read_uint();
  let w = stdin.read_uint();

  let b = vec::to_mut(vec::from_elem(w*h, ""));
  uint::range(0u, h) {|i|
    uint::range(0u, w){|j|
      b[i*w+j] = str::from_char(stdin.read_valid_char());
    }
  }
  
  ret {
    mut index: 0u, 
    w: w, 
    h: h, 
    board: vec::from_mut(b)
  };
}

fn solve(in: input) -> str {
  in;
  log(error, #fmt("idx.%u start solved", in.index));
  ret "";
}

fn main() {
  let stdin = io::stdin();
  let n = stdin.read_uint();
  let mut inputs : [input] = [];
  let mut outputs = vec::to_mut(vec::from_elem(n, ""));

  let port = comm::port();
  let chan = comm::chan(port);
  
  uint::range(0u, n) {|i| 
    let in = fetch(stdin);
    in.index = i;
    inputs += [in];
    
    task::spawn{||
      build_actor(in, chan);
    }
  }
  
  iter::repeat(n) {||
    alt check comm::recv(port){
     solved((idx, res)) {
      outputs[idx] = res;
     }
     _ {
      log(error, "what?");
     }
    }
  }
  
  vec::iteri(outputs){|i, res|
    io::println(#fmt("Case #%u: %s", i+1u, res));
  }
}

fn build_actor(in: input, chan: comm::chan<out_status>) {
  let res = solve(in);
  comm::send(chan, solved((in.index, res)));
}