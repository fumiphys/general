all: fibonacci.cpp
	@echo -- compile start --
	g++ fibonacci.cpp -o fibonacci
	g++ fibonacci.cpp -o fibonaccio3 -O3
	@echo -- compile end --
	@echo [C++]
	@./fibonacci
	@echo [C++ -O3]
	@./fibonaccio3
	@echo [python3]
	@python3 fibonacci.py

clean:
	rm -f fibonacci fibonaccio3
