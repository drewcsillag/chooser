(ql:quickload "iterate")
(defpackage chooser
  (:use "COMMON-LISP" "ITERATE")
)
(in-package :chooser)

(defstruct ebox
 (e (list nil) :type list))

(defun append-ebox (eb nl)
  (setf (ebox-e eb) (append (ebox-e eb) (list nl))))
(defun length-ebox (eb)
  (length (ebox-e eb)))
(defun popfront-ebox (eb)
  (let ((head (car (ebox-e eb)))
	(rest (cdr (ebox-e eb))))
    (setf (ebox-e eb) rest)
    head))

(defstruct chooser
  (executions (make-ebox) :type ebox)
  (index 0 :type fixnum)
  (prechosen (error "no current passed") :type list)
  (new-choices nil :type list)
  )
(defun add-execution (c e)
  (append-ebox (chooser-executions c) e))

(defun choose-index (c num-args)
;;  (format t "index ~A ~%" (chooser-index c))
;;  (format t "prechosen length ~A~%" (length (chooser-prechosen c)))
  (if (< (chooser-index c) (length (chooser-prechosen c)))
      (let ((retind (nth (chooser-index c) (chooser-prechosen c))))
	(setf (chooser-index c) (+ 1 (chooser-index c)))
	retind)
      (progn
;;	(format t "---------~%going to queue up executions and return something~%")
	(iterate (for i from 1 to (- num-args 1))
;;	  (format t "doing item ~A~%" i)
;;	  (format t "prechosen ~A~%new ~A~%i ~A~%~%"
;;		  (chooser-prechosen c)
;;		  (chooser-new-choices c)
;;		  (list i))

	  (let ((new-execution (append  (chooser-prechosen c) (chooser-new-choices c) (list i))))
	    (add-execution c new-execution)))
	(setf (chooser-new-choices c) (append (chooser-new-choices c) '(0)))
	0
	 )))



(defun run_chooser (fn)
  (let ((executions (make-ebox)))
    (iter
      ;;(for i from 1 to 2)
      (while (> (length-ebox executions) 0))
;;      (format t "--------------------~%RUNNING CHOICE~%")
      (let ((current (popfront-ebox executions)))
;;	(format t "current execution --> ~A~%" current)
	(funcall fn (make-chooser
		     :executions executions :prechosen current)))
    
      )
    executions
    )
  )

(defun simpler-test (c)
  (format t "SIMPLER TEST ~A~%" (choose-index c 2)))
(run_chooser #'simpler-test)

(defun simpler-test2 (c)
  (format t "SIMPLER TEST B) ~A ~A~%" (choose-index c 2) (choose-index c 2)))
(run_chooser #'simpler-test2)






(defvar q (make-ebox))
(setq q (make-ebox))
q
(append-ebox q '(1 2 3))
(popfront-ebox q)
(length-ebox q)
