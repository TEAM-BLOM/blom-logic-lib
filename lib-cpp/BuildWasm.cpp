#include <iostream>
#include <cstddef>
#include "Check/Check.h"
#include "MemoryPool/MemoryPool.h"
#include "Board.h"

const size_t POOL_SIZE = 1024 * 1024;
char memory_pool[POOL_SIZE];
size_t current_offset = 0;

extern "C" {
    MemoryPool boardMemoryPool;
    MemoryPool checkMemoryPool;

    Check* Init(int** _board, int row, int col, int x, int y) {
        if (_board == nullptr || row <= 0 || col <= 0) {
            printf("Invalid board or dimensions\n");
            return nullptr;
        }
        size_t boardSize = sizeof(Board);
        boardMemoryPool.Init(boardSize);

        size_t checkSize = sizeof(Check);
        checkMemoryPool.Init(checkSize);
        Board* board = static_cast<Board*>(boardMemoryPool.Allocate(boardSize));
        if (board != nullptr) {
            new (board) Board(row, col, _board);
        } else {
            return nullptr;
        }

        Check* check = static_cast<Check*>(checkMemoryPool.Allocate(checkSize));
        if (check != nullptr) {
            new (check) Check(board, x, y);
        } else {
            return nullptr;
        }

        return check;
    }

    bool CheckWin(Check* check) {
        if (check == nullptr) {
            printf("Invalid check pointer\n");
            return false;  // 잘못된 포인터
        }
        return check->check_win();
    }

    void Cleanup(Check* check) {
        if (check != nullptr) {
            check->~Check();  // 소멸자 호출
            checkMemoryPool.Deallocate(check, sizeof(Check));  // 메모리 풀에서 해제
        }
    }

    void CleanupBoard(Board* board) {
        if (board != nullptr) {
            board->~Board();  // 소멸자 호출
            boardMemoryPool.Deallocate(board, sizeof(Board));  // 메모리 풀에서 해제
        }
    }
}



// dev
/* using namespace std;

int main() {
    int** test = new int*[15];
    for (int i = 0; i < 15; i++) {
        test[i] = new int[15];
    }

    for (int i = 0; i < 15; i++) {
        for (int j = 0; j < 15; j++) {
            test[i][j] = 0;
        }
    }

    test[7][7] = -1;
    test[6][6] = -1;
    test[5][5] = -1;
    test[4][4] = -1;
    test[3][3] = -1;
    test[2][2] = 1;

    Check* check = Init(test, 15, 15, 4, 4);
    cout << CheckWin(check) << "\n";
} */