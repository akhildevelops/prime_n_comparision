libprime.so: prime.c
	$(CC) -Wall -Werror -Ofast -fpic -shared -o$@ $<

main: main.c libprime.so
	$(CC) -I. -L. -Ofast -o$@ $< -lprime

test: test.c
	$(CC) -o$@ $<

clean:
	rm main libprime.so
