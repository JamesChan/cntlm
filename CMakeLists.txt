cmake_minimum_required(VERSION 3.1)

set(PROJECT_NAME cntlm)
project(${PROJECT_NAME} C)
set(CMAKE_C_STANDARD 99)

set(SRCS
    utils.c
    utils.h
    ntlm.c
    ntlm.h
    xcrypt.c
    xcrypt.h
    config.c
    config.h
    socket.c socket.h
    acl.o
    acl.h
    auth.c
    auth.h
    http.c
    http.h
    forward.c
    forward.h
    direct.c
    direct.h
    scanner.c
    scanner.h
    pages.c
    pages.h
    main.c)

add_executable(${PROJECT_NAME} ${SRCS})
target_compile_definitions(${PROJECT_NAME} PRIVATE -D__BSD_VISIBLE -D_ALL_SOURCE -D_XOPEN_SOURCE=600 -D_POSIX_C_SOURCE=200112 -D_ISOC99_SOURCE -D_REENTRANT -D_BSD_SOURCE -DVERSION=\"0.94\")

