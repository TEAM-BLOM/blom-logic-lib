#include "MemoryPool.h"

MemoryPool::MemoryPool() 
    : canAllocate(0), ptrToAllocate(0), page(nullptr), elementSize(0) {}

MemoryPool::~MemoryPool() {
    if (page) {
        free(page);
        page = nullptr;
    }
}

void MemoryPool::Init(size_t elementSize) {
    this->elementSize = elementSize;
    InitPage();
}

void MemoryPool::InitPage() {
    if (!page) {
        page = static_cast<unsigned char *>(malloc(elementSize * MaxCount));
        if (!page) {
            throw std::bad_alloc();
        }
        canAllocate = MaxCount;
        ptrToAllocate = 0;
        
        unsigned char* p = page;
        for (int i = 0; i < MaxCount; i++) {
            *p = i + 1;
            p += elementSize;
        }
    }
}

void* MemoryPool::Allocate(size_t size) {
    if (size != elementSize) {
        throw std::invalid_argument("Requested size does not match element size.");
    }

    if (canAllocate == 0) {
        return nullptr; // 할당할 메모리가 없음
    }

    unsigned char* toAllocate = page + (ptrToAllocate * elementSize);
    ptrToAllocate = *toAllocate;
    canAllocate--;

    return static_cast<void*>(toAllocate);
}

void MemoryPool::Deallocate(void *ptr, size_t size) {
    if (size != elementSize) {
        throw std::invalid_argument("Deallocation size does not match element size.");
    }

    if (canAllocate >= MaxCount) {
        return; // 더 이상 해제할 메모리가 없음
    }

    int offset = static_cast<int>((static_cast<unsigned char*>(ptr) - page) / elementSize);
    *(static_cast<unsigned char*>(ptr)) = static_cast<unsigned char>(ptrToAllocate);
    ptrToAllocate = offset;
    canAllocate++;
}