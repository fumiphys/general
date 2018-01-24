all: fibonacci.cpp
	@echo -- compile start --
	g++ fibonacci.cpp -o fibonacci
	g++ fibonacci.cpp -o fibonaccio3 -O3
	javac Fibonacci.java
	@cd nodejs_nan
	npm install
	@cd ..
	@echo -- compile end --
	@echo [C++]
	@./fibonacci
	@echo [C++ -O3]
	@./fibonaccio3
	@echo [python3]
	@python3 fibonacci.py
	@echo [java]
	@java Fibonacci
	@nodejs nodejs_nan/fibonacci.js

clean:
	rm -f fibonacci fibonaccio3 Fibonacci.class
