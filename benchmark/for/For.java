public class For{
  public static void main(String args[]){
    int N = 1000000;
    long sum = 0;
    long start = System.nanoTime();
    for(int i = 0; i < N; i++){
      sum += i;
    }
    long time = System.nanoTime() - start;
    System.out.println("java "+sum+" "+time/1000000000.+"sec.");
  }
}
