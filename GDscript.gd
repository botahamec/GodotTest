extends Label

# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var total: float
var number: int

func _process(delta):
	if number > 25: return
	number += 1
	total += delta
	if number == 25:
		text = "GDscript: " + str(total) + "secs"
		number += 1
	else:
		text = "GDscript: " + str(fib(number))
		print(number)

func fib(num: int):
	if num < 2: return num
	return fib(num - 1) + fib(num - 2)