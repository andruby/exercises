# runtime 4.8s with ruby 2.1.0p0, 4.0s with ruby 2.2.0p0
primes = []
x = 1
begin
  x += 1
  primes << x unless primes.any? { |prime| x % prime == 0 }
end until primes.length == 10000
puts primes.join(' ')
