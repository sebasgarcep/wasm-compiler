(module
  (import "imports" "print" (func $i (param i64)))
  (func $j
    i64.const 100
    call $i
  )
  (start $j)
)