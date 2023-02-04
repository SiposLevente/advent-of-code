#include <iostream>
#include "Round.hpp"

Round::Round(Shape ownShape, Shape opponentShape) : ownShape(ownShape), opponentShape(opponentShape)
{
}

Round::~Round()
{
}

State Round::CompareShapes()
{
    State returnState = Win;
    if (this->ownShape == this->opponentShape)
    {
        returnState = Draw;
    }
    else if ((this->ownShape == Paper && this->opponentShape == Scissor) || (this->ownShape == Scissor && this->opponentShape == Rock) || (this->ownShape == Rock && this->opponentShape == Paper))
    {
        returnState = Lose;
    }

    return returnState;
}

Shape Round::GetOwnShape()
{
    return this->ownShape;
}

Shape Round::GetOpponentShape()
{
    return this->opponentShape;
}