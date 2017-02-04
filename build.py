#!/bin/python
import subprocess
import os
import shutil
import sys

def transform_mem(fin, fout):

	ifd = open(fin, 'rb')
	data = ifd.read()
	ifd.close()

	ofd = open(fout, 'w')

	ofd.write('module.exports = "')

	print 'transforming', len(data), len(data) * 4

	for x in xrange(0, len(data)):
		b = hex(ord(data[x]))[2:]
		if len(b) < 2:
			ofd.write('\\x0' + b)
		else:
			ofd.write('\\x' + b)

	ofd.write('";')

	ofd.close()

def modifyasm(fin, fout):
	ifd = open(fin, 'r')
	ofd = open(fout, 'w')

	lines = ifd.readlines()

	ifd.close()

	ofd.write('let Module = {};\n')

	for line in lines:
		ofd.write(line)

	ofd.write('module.exports = Module.asm;\n');

	ofd.close()

def check_env():
	print '------ checking environment -------'
	def check(cmd, msg):
		success = True
		try:
			subprocess.check_call(cmd, shell=True)
		except subprocess.CalledProcessError as e:
			print msg
			success = False

		return success
	return check('emcc -v', 'Could not execute emcc (Emscripten).') and check('cargo --version', 'Could not execute cargo (Rust).')

def build(inp, outp, md, emop):
	if emop:
		emop = '2'
	else:
		emop = '0'

	print '------ building ------'
	#os.environ['RUSTFLAGS'] = '-C target-feature="-sse -sse2 -sse3 -sse4.1 -sse4a -ssse3"]'
	#subprocess.check_call('cargo build%s --target asmjs-unknown-emscripten' % md, shell=True) # --emit llvm-ir --crate-type lib test.rs
	subprocess.check_call('rustc -O --crate-type rlib ./src/lib.rs -o ./libscreeps_rust_code.rlib --target asmjs-unknown-emscripten', shell=True)

	exp = "['_game_tick', '___allocate', '___deallocate', '_bitshift64Lshr']"
	eexp = "['setTempRet0', 'getTempRet0']"
	subprocess.call('emcc --separate-asm -v %s -s NO_EXIT_RUNTIME=1 --bind -s EXTRA_EXPORTED_RUNTIME_METHODS="%s" -s STACK_OVERFLOW_CHECK=0 -s EXPORTED_FUNCTIONS="%s" -s ONLY_MY_CODE=0 --separate-asm -s SWAPPABLE_ASM_MODULE=1 -O%s' % (inp, eexp, exp, emop), shell=True)

	try:
		os.mkdir('./output')
	except:
		pass

	transform_mem('./a.out.js.mem', './output/rust.mem.js')
	modifyasm('./a.out.asm.js', './output/rust.asm.js')

	#os.remove('./a.out.asm.js')
	#os.remove('./a.out.js')
	#os.remove('./a.out.js.mem')

	shutil.copyfile('./main.js', './output/main.js')
	shutil.copyfile('./rust.boot.js', './output/rust.boot.js')

def main(debug, emscripten_optimize):
	if check_env() is False:
		return

	if debug:
		md = ''
		sdir = 'debug'
	else:
		md = ' --release'
		sdir = 'release'

	build(
		'./libscreeps_rust_code.rlib', # './target/asmjs-unknown-emscripten/%s/libscreeps_rust_code.rlib' % sdir, 
		'./rust', 
		md,
		emscripten_optimize
	)

main(False, True)