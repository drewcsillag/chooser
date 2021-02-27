(ql:quickload "iterate")
(defpackage chooser
  (:use "COMMON-LISP" "ITERATE")
)
(in-package :chooser)

(defstruct chooser
  "structure for holding chooser state for the current execution"
  (executions (error "no executions passed") :type list)
  (current (error "no current passed") :type list)
  (index 0 :type fixnum)
  (prechosen nil :type list)
  (new-choices nil :type list)
  )


(declaim (ftype (function (chooser fixnum) fixnum) choose-index))
(defun choose-index (c num-args)
  (check-type num-args fixnum)
  (check-type c chooser)
  (format t "index ~A ~%" (chooser-index c))
  (format t "prechosen ~A~%" (length (chooser-prechosen c)))
  (if (< (chooser-index c) (length (chooser-prechosen c)))
      (let ((retind (nth (chooser-index c) (chooser-prechosen c))))
	(setf (chooser-index c) (+ 1 (chooser-index c)))
	retind)
      (progn
	(iterate (for i from 1 to num-args)
	  (nconc (chooser-executions c)
		 (append (chooser-prechosen c) (chooser-new-choices c) (list i))))
	(setf (chooser-new-choices c) (append (chooser-new-choices c) '(0)))
	0)))

(defun run_chooser (fn)
  (let ((executions '(nil)))
    (iter
      (while (> (length executions) 0))
      (let ((current (car executions))
	    (newexec (cdr executions)))
	(setq executions newexec)
	(funcall fn (make-chooser
		     :executions executions :current current))))))

(defun binary-test (c)
  (check-type c chooser::chooser)
  (let ((a (choose-index c 2))
	(b (choose-index c 2))
	(c (choose-index c 2)))
	
  (format t "~A ~A ~A ~%" a b c)))

(run_chooser #'chooser::binary-test)

(let ((executions nil)
      (current nil))
(setf qq (make-chooser
		     :executions executions :current current)))
      
(defvar qq (make-chooser
		     :executions executions :current current))
(chooser::choose-index qq 2)
qq
