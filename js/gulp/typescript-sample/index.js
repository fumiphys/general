var foo = /** @class */ (function () {
    function foo(b, c) {
        this.b = b;
        this.c = c;
        this.a = b + c;
    }
    return foo;
}());
function printnum(x) {
    document.body.innerHTML = x;
}
var foo1 = new foo('Hello ', 'World');
printnum(foo1.a);
