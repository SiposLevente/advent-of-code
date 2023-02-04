enum Shape
{
    Rock = 1,
    Paper,
    Scissor
};

enum State
{
    Lose = 0,
    Draw = 3,
    Win = 6,
};

class Round
{
private:
    Shape ownShape;
    Shape opponentShape;

public:
    State CompareShapes();
    Shape GetOwnShape();
    Shape GetOpponentShape();
    Round(Shape ownShape, Shape opponentShape);
    ~Round();
};