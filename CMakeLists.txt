#***********************************************************************
#
#        CMakeLists.txt for Akonadi command line helper
#
#        Created  on: Dec  19, 2025
#        Modified on:
#        Author: Dr. Christoph Pospiech
#
#***********************************************************************

cmake_minimum_required(VERSION 3.20.0)

project(akonadi_helpers VERSION 0.1.0 LANGUAGES CXX)

# Cmake modules/macros are in a subdirectory to keep this file cleaner
set(CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/cmake)

enable_language(CXX)

######################################
# compiler tests                     #
######################################

include(CheckCXXCompilerFlag)

######################################
# Go to the helper directory         #
######################################

add_subdirectory(helper)
