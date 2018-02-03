#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/wait.h>

#define MAX_ARGS (20)
#define MAX_LEN (500)

int main(){
  int argc = 0;
  char input[MAX_LEN];
  char *argv[MAX_ARGS], *cp;
  int status;
  const char *delimiter = " \t\n";

  while(1){
    printf(">> ");
    if(fgets(input, sizeof(input), stdin) == NULL){
      printf("Bye.\n");
      exit(0);
    }

    cp = input;
    for(argc = 0; argc < MAX_ARGS; argc++){
      if((argv[argc] = strtok(cp, delimiter)) == NULL){
        break;
      }
      cp = NULL;
    }

    if(strcmp(argv[0], "exit") == 0){
      exit(0);
    }

    if(strcmp(argv[0], "cd") == 0){
      if(argv[1]){
        if(chdir(argv[1]) != 0){
          fprintf(stderr, "error while change directory\n");
        }
      }else{
      }
    }else{
      pid_t pid = fork();
      if(pid < 0){
        perror("error while fork");
        exit(-1);
      }else if(pid == 0){
        execvp(argv[0], argv);
        perror("error at child process");
        exit(-1);
      }else{
        wait(&status);
      }
    }
  }
}
