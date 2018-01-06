class foo{
  a: string;
  constructor(public b: string, public c: string){
    this.a = b + c;
  }
}

function printnum(x: string){
  document.body.innerHTML = x;
}

let foo1 = new foo('Hello ', 'World');
printnum(foo1.a);
