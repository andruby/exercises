#include <stdio.h>

int main() {
  int primes[10000];
  int x = 2;
  int length;
  int i;
  int isPrime;
  for (length = 0; length < 10000; x++) {
    isPrime = 1;
    for (i = 0; i < length; i++) {
      if (x % primes[i] == 0) {
        isPrime = 0;
        break;
      }
    }
    if (isPrime) {
      primes[length] = x;
      length = length + 1;
    }
  }

  for (i = 0; i < length; i++) {
    printf("%d ", primes[i]);
  }
  return 0;
}
