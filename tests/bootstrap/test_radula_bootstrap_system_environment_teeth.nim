# Copyright (c) 2018-2024, Firas Khalil Khana
# Distributed under the terms of the ISC License

import std/os

import
  ../../src/bootstrap,
  ../../src/constants,
  ../../src/genome

radula_genome_environment()

radula_bootstrap_system_environment_teeth()

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

doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_ARCHIVER) == "gcc-ar"
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_ASSEMBLER) == "as"
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_C_COMPILER) == "gcc"
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_C_PREPROCESSOR) == "gcc -E"
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_CXX_COMPILER) == "g++"
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_CXX_PREPROCESSOR) == "g++ -E"
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_HOST_C_COMPILER) == "gcc"
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_NAMES) == "gcc-nm"
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_OBJECT_COPY) == "objcopy"
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_OBJECT_DUMP) == "objdump"
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_RANDOM_ACCESS_LIBRARY) == "gcc-ranlib"
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_READ_ELF) == "readelf"
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_SIZE) == "size"
doAssert getEnv(RADULA_ENVIRONMENT_TOOTH_STRIP) == "strip"
