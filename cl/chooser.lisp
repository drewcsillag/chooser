(ql:quickload "iterate")
(ql:quickload "alexandria")

(defpackage chooser
  (:use "COMMON-LISP" "ITERATE" "ALEXANDRIA")
)
(in-package :chooser)

(defstruct chooser
  (append-execution (error "no append-execution passed") :type (function (list)))
  (index 0 :type fixnum)
  (prechosen (error "no current passed") :type list)
  (new-choices nil :type list)
  )

(declaim (ftype (function (chooser fixnum) fixnum) choose-index))
(defun choose-index (c num-args)
  "Given a chooser C and NUM-ARGS returns a number between 0 and NUM-ARGS-1, inclusive."
  (if (< (chooser-index c) (length (chooser-prechosen c)))
      (let ((retind (nth (chooser-index c) (chooser-prechosen c))))
	(incf (chooser-index c))
	(return-from choose-index retind)))
  (iter (for i from 1 to (- num-args 1))
    (let ((new-execution (append (chooser-prechosen c) (chooser-new-choices c) (list i))))
      (funcall (chooser-append-execution c) new-execution)))
  (appendf (chooser-new-choices c) '(0))
  0)

;; in the ideal world the t's would be enforce to be the same thing
;; by doing some kind of capture 
(declaim (ftype (function (list fixnum (function (t))) t) remove-nth))				
(defun remove-nth (l ind setter)
  "Calls SETTER with the contents of L with the item at index IND removed"
  (funcall setter (remove-if (constantly t) l :start ind :count 1)))

(declaim (ftype (function (chooser list (function (t))) t) chooser-pick))
(defun chooser-pick (c items setter)
  "pick an item from ITEMS using C and call SETTER with ITEMS with the chosen item removed"
  (let* ((ind (choose-index c (length items)))
	 (ret (nth ind items)))
    (remove-nth items ind setter)
    ret))

(defun chooser-choose (c items)
  "Returns a chosen item from ITEMS based on chooser C"
  (let ((ind (choose-index c (length items))))
    (nth ind items)))
	 
(declaim (ftype (function ((function (t)))) run_chooser))
(defun run_chooser (fn)
  "Runs the chooser loop on FN"
  (let* ((executions '(nil)))
    (iter (while (> (length executions) 0))
      (funcall fn (make-chooser
		   :append-execution (lambda (e) (appendf executions (list e)))
		   :prechosen (pop executions))))))

(defun simpler-test (c)
  (format t "SIMPLER TEST ~A~%" (choose-index c 2)))
(run_chooser #'simpler-test)

(defun simpler-test2 (c)
  (format t "SIMPLER TEST B) ~A ~A~%" (chooser-choose c '("1" "2" "3")) (chooser-choose c '("1" "2" "3"))))
(run_chooser #'simpler-test2)

(defun solve-magic-square (c consumer)
  (let ((remaining '(1 2 3 4 5 6 7 8 9))
	(square nil))
    (labels (
	     (neq15 (i1 i2 i3) ;; not equal sum to 15 by index
	       (/= 15 (+ (nth i1 square) (nth i2 square) (nth i3 square))))
	     (addpick () ;; pick a choice and add to square
	       (appendf square (list (chooser-pick c remaining #'writeremaining))))
	     (writeremaining (x) ;; what chooser-pic uses to update remaining
	       (setf remaining x))
	     (checkbail (i1 i2 i3) ;; if the sum isn't 15, bail
	       (if (neq15 i1 i2 i3)
		   (return-from solve-magic-square)))
	     )
      ;; 0 1 2
      ;; 3 4 5
      ;; 6 7 8
      (addpick) ;0
      (addpick) ;1
      (addpick) ;2
      (checkbail 0 1 2) ;across row 1
      (addpick) ;3
      (addpick) ;4 
      (addpick) ;5
      (checkbail 3 4 5) ;across row 2
      (addpick) ;6 
      (checkbail 0 3 6) ;down row 1
      (checkbail 2 4 6) ;diagonal up to right
      (addpick) ;7
      (checkbail 1 4 7) ;down row 2
      (addpick) ;8
      (checkbail 6 7 8) ;across row 3
      (checkbail 2 5 8) ;down row 3
      (checkbail 0 4 8) ;diagonal down to right
      (funcall consumer square)
      )))

(defvar sol-count 0)
(setq sol-count 0)
(defun consume (square)
  (format t "Square ~A~%" square)
  (setf sol-count (+ 1 sol-count)))

(run_chooser (lambda (c) (solve-magic-square c #'consume)))
sol-count

(run_chooser (lambda (c) (format t "~A ~A ~A~%" (chooser-choose c '(0 1)) (chooser-choose c '(0 1)) (chooser-choose c '(0 1)))))
