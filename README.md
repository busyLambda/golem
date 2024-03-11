# GOLEM

## Description
Garbage collected porgramming language that targets C

## Examples

```gm
addTwo : (int, int) -> int
addTwo = (a, b) do a + b

/*
Compiles to:

int addTwo(int a,int b){return a+b;}
*/

main : () -> void
main = () {
    a := 1
    b := 2
    c := addTwo(a, b)
    print(c) 
    
    list := [1, 2, 3, 4, 5]
    
    f := fn (x) do x =* 2

    mapped := map list <- f
    iprint(mapped)
}
```