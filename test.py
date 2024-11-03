import pyjsmath as pyjs
import tcrutils as tcr
from tcrutils import ass, c


def test_ass():
	ass(pyjs.addf(1.2, 3.4), (1.2 + 3.4))
	ass(pyjs.subf(1.2, 3.4), (1.2 - 3.4))
	ass(pyjs.mulf(1.2, 3.4), (1.2 * 3.4))
	print()
	ass(pyjs.addi(1, 3), (1 + 3))
	ass(pyjs.subi(1, 3), (1 - 3))
	ass(pyjs.muli(1, 3), (1 * 3))
	print()
	ass(pyjs.truediv(1.2, 3.4), (1.2 / 3.4))
	ass(pyjs.floordiv(1.2, 3.4), (1.2 // 3.4))
	print()
	ass(pyjs.modulof(1.2, 3.4), (1.2 % 3.4))
	ass(pyjs.moduloi(5, 4), (5 % 4))
	print()
	ass(pyjs.powf(1.2, 3.4), (1.2 ** 3.4))
	ass(pyjs.powi(5, 4), (5 ** 4))

def test_range():
	for i in range(1000):
		c(f'10**{i} + 10**{i} =', pyjs.addf(10**i, 10**i))

# test_range()
print()
test_ass()
