/*Runtime 0.6s with Go 1.2*/
package main
import "fmt"

func main() {
  var primes [10000]int
  var x int = 2
  for length := 0; length < 10000; x++ {
    isPrime := true
    for i := 0; i < length; i++ {
      if x % primes[i] == 0 {
        isPrime = false
        break
      }
    }
    if isPrime {
      primes[length] = x
      length = length + 1
    }
  }
  fmt.Println(primes)
}
