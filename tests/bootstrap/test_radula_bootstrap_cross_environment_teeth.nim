# Copyright (c) 2018-2024, Firas Khalil Khana
# Distributed under the terms of the ISC License

import std/[
  os,
  strutils
]

import
  ../../src/bootstrap,
  ../../src/constants,
  ../../src/genome

radula_bootstrap_environment()

radula_genome_environment()

radula_bootstrap_cross_environment_teeth()

echo "CROSS_COMPILE  :: ", getEnv(RADULA_ENVIRONMENT_CROSS_COMPILE)

echo ""

echo "AR             :: ", getEnv(RADULA_ENVIRONMENT_TOOTH_ARCHIVER)
echo "AS             :: ", getEnv(RADULA_ENVIRONMENT_TOOTH_ASSEMBLER)
echo "CC             :: ", getEnv(RADULA_ENVIRONMENT_TOOTH_C_COMPILER)
echo "CPP            :: ", getEnv(RADULA_ENVIRONMENT_TOOTH_C_PREPROCESSOR)
echo "CXX            :: ", getEnv(RADULA_ENVIRONMENT_TOOTH_CXX_COMPILER)
echo "CXXCPP         :: ", getEnv(RADULA_ENVIRONMENT_TOOTH_CXX_PREPROCESSOR)
echo "HOSTCC         :: ", getEnv(RADULA_ENVIRONMENT_TOOTH_HOST_C_COMPILER)
echo "NM             :: ", getEnv(RADULA_ENVIRONMENT_TOOTH_NAMES)
echo "OBJCOPY        :: ", getEnv(RADULA_ENVIRONMENT_TOOTH_OBJECT_COPY)
echo "OBJDUMP        :: ", getEnv(RADULA_ENVIRONMENT_TOOTH_OBJECT_DUMP)
echo "RANLIB         :: ", getEnv(RADULA_ENVIRONMENT_TOOTH_RANDOM_ACCESS_LIBRARY)
echo "READELF        :: ", getEnv(RADULA_ENVIRONMENT_TOOTH_READ_ELF)
echo "SIZE           :: ", getEnv(RADULA_ENVIRONMENT_TOOTH_SIZE)
echo "STRIP          :: ", getEnv(RADULA_ENVIRONMENT_TOOTH_STRIP)

doAssert getEnv(RADULA_ENVIRONMENT_CROSS_COMPILE).endsWith("-")

doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_ARCHIVER).endsWith("gcc-ar")
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_ASSEMBLER).endsWith("as")
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_C_COMPILER).endsWith("gcc")
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_C_PREPROCESSOR).endsWith("gcc -E")
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_CXX_COMPILER).endsWith("g++")
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_CXX_PREPROCESSOR).endsWith("g++ -E")
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_HOST_C_COMPILER) == "gcc"
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_NAMES).endsWith("gcc-nm")
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_OBJECT_COPY).endsWith("objcopy")
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_OBJECT_DUMP).endsWith("objdump")
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_RANDOM_ACCESS_LIBRARY).endsWith("gcc-ranlib")
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_READ_ELF).endsWith("readelf")
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_SIZE).endsWith("size")
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_STRIP).endsWith("strip")
