#include <stdio.h>
#include <stdlib.h>

struct Fibonacci_Numbers {
	int Current;
	int Previous[2];
};

int main(int argc, char** argv) {
	if (argc < 2) { // ensure argument was given
		printf("No arguments supplied.\n");
		return 0;
	}

	int num = atoi(argv[1]);
	if (num == 0) { // ensure argument give was number
		printf("Argument supplied was not number!\n");
		return 1;
	}

	struct Fibonacci_Numbers FN;	
	FN.Current = 0;
	FN.Previous[0] = 1;
	FN.Previous[1] = 2;
	printf("Current: 1\n");
	printf("Current: 2 (2)\n");

	int sum = 2;
	while (1) {
		FN.Current = FN.Previous[0] + FN.Previous[1];	// 3, 1, 2 | 5, 2, 3 | 8, 3, 5
		FN.Previous[0] = FN.Previous[1]; 				// 3, 2, 2 | 5, 3, 3 | 8, 5, 5
		FN.Previous[1] = FN.Current; 					// 3, 2, 3 | 5, 3, 5 | 8, 5, 8
	
		if (FN.Current > num) {
			printf("Reached %d, too high, breaking loop\n", sum);
			break;
		}

		printf("Current: %d", FN.Current);

		if (FN.Current % 2 == 0) {
			sum += FN.Current;
			printf(" (%d)\n", sum);
		}
		else {
			printf("\n");
		}
	}
	printf("Sum of even fibonacci number terms below argument: %d\n", sum);
	return 0;

}

