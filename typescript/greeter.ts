interface Person{
  firstname: string;
  lastname: string;
  }

class User{
  fullname: string;
  constructor(public firstname: string,
  public lastname: string){
    this.fullname = firstname + " " + lastname;
    }
}

function greeter(person: Person){
  return "hello " + person.firstname + " " + person.lastname;
}

let user = new User("fumi", "phys");

document.body.innerHTML = greeter(user);
