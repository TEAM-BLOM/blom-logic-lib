#include <iostream>
#include "MemoryPool/MemoryPool.h"

int main() {
    
    MemoryPool pool;
    pool.Init(32); 

    // 메모리 할당 테스트
    void* ptr1 = pool.Allocate(32);
    if (ptr1 != nullptr) {
        std::cout << "ptr1 할당 성공!" << std::endl;
    } else {
        std::cout << "ptr1 할당 실패!" << std::endl;
    }

    void* ptr2 = pool.Allocate(32);
    if (ptr2 != nullptr) {
        std::cout << "ptr2 할당 성공!" << std::endl;
    } else {
        std::cout << "ptr2 할당 실패!" << std::endl;
    }

    // 메모리 해제 테스트
    pool.Deallocate(ptr1, 32);
    std::cout << "ptr1 해제 완료!" << std::endl;

    pool.Deallocate(ptr2, 32);
    std::cout << "ptr2 해제 완료!" << std::endl;

    // 여러 번의 할당/해제 테스트
    void* ptrArray[10];
    for (int i = 0; i < 10; ++i) {
        ptrArray[i] = pool.Allocate(32);
        if (ptrArray[i] != nullptr) {
            std::cout << "ptrArray[" << i << "] 할당 성공!" << std::endl;
        } else {
            std::cout << "ptrArray[" << i << "] 할당 실패!" << std::endl;
        }
    }

    for (int i = 0; i < 10; ++i) {
        pool.Deallocate(ptrArray[i], 32);
        std::cout << "ptrArray[" << i << "] 해제 완료!" << std::endl;
    }

    // 모든 메모리 해제 후 다시 할당
    void* ptr3 = pool.Allocate(32);
    if (ptr3 != nullptr) {
        std::cout << "ptr3 할당 성공!" << std::endl;
    } else {
        std::cout << "ptr3 할당 실패!" << std::endl;
    }

    return 0;
}
