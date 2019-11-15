package com.leviathan.boiler_lib

external fun hello(to: String): String

fun loadBoilerLib() {
    System.loadLibrary("boiler_rust")
}