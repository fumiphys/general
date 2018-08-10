all: fibonacci.cpp
	@echo -- compile start --
	g++ fibonacci.cpp -o fibonaccio
	g++ fibonacci.cpp -o fibonaccio3 -O3
	javac Fibonacci.java
	# @cd nodejs_nan
	# npm install
	# @cd ..
	rustc -O fibonacci.rs
	@echo -- compile end --
	@echo [C++]
	@./fibonaccio
	@echo [C++ -O3]
	@./fibonaccio3
	@echo [python3]
	# @python3 fibonacci.py
	@echo [java]
	@java Fibonacci
	@echo [Rust]
	@./fibonacci
	# @echo [nodejs + C++]
	# @node nodejs_nan/fibonacci.js
	@echo [nodejs]
	@node fibonacci.js

clean:
	rm -f fibonaccio fibonaccio3 Fibonacci.class fibonacci
