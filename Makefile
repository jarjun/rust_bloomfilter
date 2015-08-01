
bin: 
	mkdir bin

bloom: bin
	rustc bloomfilter.rs -o bin/bloom

test: bin
	rustc --test test_bloomfilter.rs -o bin/test

clean:
	rm -rf bin/


