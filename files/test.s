start: Jump 2, then-1, 500
Jump 500, 400, 450
Jump then, 400, 500
Jump 310, 410, 510
6
F data, data, start
then: AddIS36Sat data, data, start+1
AddIS36Sat data, data, start+2
Jump next, start, start
P data, data+1, out
next: Q data, data+1, out+1
Jump 30, 0, 0
data: 5
2
out: 3
