var User = /** @class */ (function () {
    function User(firstname, lastname) {
        this.firstname = firstname;
        this.lastname = lastname;
        this.fullname = firstname + " " + lastname;
    }
    return User;
}());
function greeter(person) {
    return "hello " + person.firstname + " " + person.lastname;
}
var user = new User("fumi", "phys");
document.body.innerHTML = greeter(user);
