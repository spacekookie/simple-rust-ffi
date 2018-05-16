cmake_minimum_required(VERSION 2.6)

add_library(greet STATIC greet.c)
target_include_directories(greet PUBLIC ".")
install(TARGETS greet DESTINATION ".")