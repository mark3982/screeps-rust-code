#!/bin/python
import subprocess
import os
import shutil

def modifyasm(fin, fout):
	ifd = open(fin, 'r')
	ofd = open(fout, 'w')

	lines = ifd.readlines()

	ifd.close()

	ofd.write('module.exports = (function(global, env, buffer) {\n')

	for x in xrange(1, len(lines)):
		ofd.write(lines[x])

	ofd.close()

def check_env():
	print '------ checking environment -------'
	def check(cmd):
		try:
			subprocess.check_call(cmd, shell=True)
		except Exception as e:
			print 'The emscripten executable emcc could not be properly executed. Check `emcc`.'
			return False

	return check('emcc -v') and check('cargo --version')

def build(inp, outp):
	print '------ building ------'
	subprocess.check_call('cargo build --release --target asmjs-unknown-emscripten', shell=True) # --emit llvm-ir --crate-type lib test.rs
	subprocess.call('emcc --separate-asm -v %s -s NO_EXIT_RUNTIME=1 --bind -s EXPORTED_FUNCTIONS="[\'_game_tick\']" -s ONLY_MY_CODE=1 --separate-asm -s SWAPPABLE_ASM_MODULE=1 -O3' % inp, shell=True)

	try:
		os.mkdir('./output')
	except:
		pass

	modifyasm('./a.out.asm.js', './output/rust.asm.js')

	os.remove('./a.out.asm.js')
	os.remove('./a.out.js')
	os.remove('./a.out.js.mem')

	shutil.copyfile('./rust.boot.js', './output/rust.boot.js')

def main():
	if check_env() is False:
		return

	build('./target/asmjs-unknown-emscripten/release/librust_screeps_code.rlib', './rust')

main()