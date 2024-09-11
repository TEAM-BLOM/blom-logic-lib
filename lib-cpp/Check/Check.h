#ifndef CHECK_WIN_H
#define CHECK_WIN_H

#include <iostream>
#include <stdexcept>
#include "../Board.h"

class Check {
    private:
        Board* board;
        bool is_win;
        Pair* current;

    public:
        Check(Board* board, int x, int y) {
            this->board = board;
            this->current = new Pair(x, y);
        }

        ~Check() {
            delete board;
            delete current;
        }

        bool check_win();
};

#endif
