function fibonacci(i){
  if(i == 0 || i == 1){
    return i;
  }else{
    return fibonacci(i-1) + fibonacci(i-2);
  }
}

var startTime = new Date();
result = fibonacci(40);
var endTime = new Date();

time = endTime - startTime;

console.log("javascript " + result + " " + time/1000.0 + "sec.")
