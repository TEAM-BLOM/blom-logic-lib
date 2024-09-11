#ifndef MEMORY_POOL_H
#define MEMORY_POOL_H

#include <cstdlib>
#include <stdexcept>

class MemoryPool {
public:
    void Init(size_t elementSize);
    void* Allocate(size_t size);
    void Deallocate(void *ptr, size_t size);

public:
    MemoryPool();
    ~MemoryPool();

private:
    int canAllocate; // 남아있는 블럭 개수
    int ptrToAllocate; // 다음에 할당할 블럭의 인덱스
    unsigned char* page; // 메모리 풀의 첫 번째 페이지

    size_t elementSize; // 각 블럭의 크기

    static const int MaxCount = 1024; // 최대 블럭 개수

    void InitPage(); // 메모리 페이지를 초기화하는 함수
};

#endif