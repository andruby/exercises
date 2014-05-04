/*Runs in 0.597s in Go 1.2*/
package main
import "fmt"

func main() {
  var primes [10000]int
  var testNumber int = 2
  for length := 0; length < 10000; testNumber++ {
    divisable := true
    for i := 0; i < length; i++ {
      if testNumber % primes[i] == 0 {
        divisable = false
        break
      }
    }
    if divisable == true {
      primes[length] = testNumber
      length = length + 1
    }
  }
  fmt.Println(primes)
}
