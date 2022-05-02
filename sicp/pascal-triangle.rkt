#lang racket
(define (pascal line column)
  (cond((or (= line 1) (= column 1) (= column line)) 1)
       (else (+ (pascal (- line 1) (- column 1)) (pascal (- line 1) column)))
       ))

(define (pascal-triangle obj-line cur-line cur-column)
  (cond ((> cur-line obj-line))
        ((> cur-column cur-line) (newline) (pascal-triangle obj-line (+ cur-line 1) 1))
        (else (display (pascal cur-line cur-column))
              (display " ")
              (pascal-triangle obj-line cur-line (+ cur-column 1)))
        )
  )

(pascal-triangle 10 1 1)