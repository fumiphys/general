all: for.cpp for.py
	@echo -- compile start --
	g++ for.cpp -o forcpp
	g++ for.cpp -o forcppo3 -O3
	@echo -- compile end --
	@echo [C++]
	@./forcpp
	@echo [C++ -O3]
	@./forcppo3
	@echo [python3]
	@python3 for.py

clean:
	rm -f forcpp forcppo3
