all: for.cpp for.py
	@echo -- compile start --
	g++ for.cpp -o forcpp
	g++ for.cpp -o forcppo3 -O3
	javac For.java
	rustc -O for.rs
	@echo -- compile end --
	@echo [C++]
	@./forcpp
	@echo [C++ -O3]
	@./forcppo3
	@echo [python3]
	@python3 for.py
	@echo [java]
	@java For
	@./for

clean:
	rm -f forcpp forcppo3 For.class
