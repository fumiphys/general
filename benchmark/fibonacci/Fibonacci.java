public class Fibonacci{
  public static int fib(int n){
    if(n == 0 || n == 1){
      return n;
    }else{
      return fib(n-1) + fib(n-2);
    }
  }
  public static void main(String args[]){
    int N = 40;
    long start = System.nanoTime();
    long fibonacci = fib(N);
    long time = System.nanoTime() - start;
    System.out.println("java "+fibonacci+" "+time/1000000000.+"sec.");
  }
}
