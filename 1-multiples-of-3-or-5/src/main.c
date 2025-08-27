#include <stdio.h>
#include <stdlib.h>

int main(int argc, char** argv) {

	//ensure argument was supplied
	if (argc < 2) {
		printf("No Arguments supplied\n");
		return 0;
	}

	// attempt conversion of second argument to integer
	int num = atoi(argv[1]);
	// num will be 0 if the ascii string was not numbers
	if (num == 0) {
		printf("Argument supplied was not number\n");
		return 1;
	}

	int sum = 0;
	for(int i = 0; i < num; i++) {
		if (i % 3 == 0 || i % 5 == 0) {
			sum += i;
		}
	}
	printf("Sum: %d\n", sum);
	return 0;
}
