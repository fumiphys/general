#include <nan.h>

int calcFibonacci(int n){
  if(n == 0 || n == 1){
    return n;
  }else{
    return calcFibonacci(n-1) + calcFibonacci(n-2);
  }
}

void Fibonacci(const Nan::FunctionCallbackInfo<v8::Value>& info){
  if(info.Length() < 1){
    Nan::ThrowTypeError("please input argument");
    return;
  }

  if(!info[0]->IsNumber()){
    Nan::ThrowTypeError("argument is int");
    return;
  }

  int n = info[0]->NumberValue();
  info.GetReturnValue().Set(Nan::New(calcFibonacci(n)));
}

void Init(v8::Local<v8::Object> exports){
  exports->Set(Nan::New("fibonacci").ToLocalChecked(),
      Nan::New<v8::FunctionTemplate>(Fibonacci)->GetFunction());
}

NODE_MODULE(fibonacci, Init);
