#include <iostream>
#include <map>
#include <set>
#include <string>
#include <sstream>
#include <cassert>
using namespace std;

class GateGame;

struct Value {
    enum ValueType { STR, CONST };
    Value() : cons(0) { }
    Value(Value&& v) : cons(v.cons), str(move(v.str)), type(v.type) {}
    uint16_t cons;
    string str;
    ValueType type;
};

Value
MaybeName(istream& istr)
{
    Value v;
    if (istr >> v.cons) {
        v.type = Value::CONST;
        return v;
    }

    istr.clear();
    v.type = Value::STR;
    istr >> v.str;
    return v;
}


struct Gate
{
    virtual ~Gate() { }
    virtual uint16_t Resolve(GateGame& env, set<string>& resolving) = 0;
    virtual bool isConstant() {
        return false;
    }
};

class GateGame
{
public:
    ~GateGame() {
        for (auto i = mEnv.begin(); i != mEnv.end(); ++i)
            delete i->second;
    }
    uint16_t Resolve(const string& name, set<string>& resolving);
    void ParseLine(const string& line);
    void Run();

    void Print();

private:
    typedef map<string, Gate*> Environment;
    Environment mEnv;
};

struct Constant : Gate
{
    Constant(uint16_t value) : mValue(value) { }
    virtual ~Constant() { }

    virtual uint16_t Resolve(GateGame& env, set<string>& resolving) {
        return mValue;
    }
    virtual bool isConstant() {
        return true;
    }

    uint16_t mValue;
};

struct Not : Gate
{
    Not(const string& rhs) : mRhs(rhs) { }
    virtual ~Not() { }

    virtual uint16_t Resolve(GateGame& env, set<string>& resolving) {
        return ~env.Resolve(mRhs, resolving);
    }

    string mRhs;
};

struct MoveFrom : Gate
{
    MoveFrom(const Value& v) : mRhs(v) { }
    virtual ~MoveFrom() { }

    virtual uint16_t Resolve(GateGame& env, set<string>& resolving) {
        if (mRhs.type == Value::CONST)
            return mRhs.cons;
        return env.Resolve(mRhs.str, resolving);
    }

    Value mRhs;
};

struct AndOr : Gate
{
    AndOr(const Value& lhs, const Value& rhs, bool isAnd)
        : mLhs(lhs), mRhs(rhs), mIsAnd(isAnd)
    { }

    virtual uint16_t Resolve(GateGame& env, set<string>& resolving) {
        uint16_t lhs = mLhs.type == Value::CONST ? mLhs.cons : env.Resolve(mLhs.str, resolving);
        uint16_t rhs = mRhs.type == Value::CONST ? mRhs.cons : env.Resolve(mRhs.str, resolving);

        return mIsAnd ? (lhs & rhs) : (lhs | rhs);
    }

    Value mLhs;
    Value mRhs;
    bool mIsAnd;
};

struct Shift : Gate
{
    Shift(Value& lhs, uint16_t howmuch, bool left)
        : mLhs(lhs), mHowmuch(howmuch), mLeft(left)
    { assert(mHowmuch); }

    virtual uint16_t Resolve(GateGame& env, set<string>& resolving) {
        uint16_t lhs = mLhs.type == Value::CONST ? mLhs.cons : env.Resolve(mLhs.str, resolving);

        return mLeft ? (lhs << mHowmuch) : (lhs >> mHowmuch);
    }

    Value mLhs;
    uint16_t mHowmuch;
    bool mLeft;
};

uint16_t
GateGame::Resolve(const string& name, set<string>& resolving)
{
    auto gate = mEnv.find(name);
    if (gate == mEnv.end()) {
        cout << "Couldn't find: " << name << '\n';
        assert(false);
    }

    if (gate->second->isConstant())
        return gate->second->Resolve(*this, resolving);

    if (resolving.find(name) != resolving.end()) {
        cout << "Circular resolve? " << name << '\n';
        assert(0);
    }

    resolving.insert(name);

    uint16_t value = gate->second->Resolve(*this, resolving);
    delete gate->second;
    mEnv[name] = new Constant(value);

    resolving.erase(name);
    return value;
}

void
GateGame::ParseLine(const string& line)
{
    istringstream istr(line);

    if (line[0] == 'N') {
        // NOT
        string rhs;
        istr >> rhs; // NOT
        istr >> rhs; // gate

        string target;
        istr >> target; // ->
        istr >> target;
        mEnv[target] = new Not(rhs);
        return;
    }

    Value lhs = MaybeName(istr);

    string op;
    istr >> op;
    if (op == "->") {
        string target;
        istr >> target;
        mEnv[target] = new MoveFrom(lhs);
        return;
    }

    Value rhs = MaybeName(istr);
    Gate* g;
    if (op == "AND" || op == "OR") {
        g = new AndOr(lhs, rhs, op == "AND");
    } else {
        assert(rhs.type == Value::CONST);
        g = new Shift(lhs, rhs.cons, op == "LSHIFT");
    }

    string target;
    istr >> target; // ->
    istr >> target;
    mEnv[target] = g;
}

void
GateGame::Run()
{
    set<string> resolving;
    for (auto i = mEnv.begin(); i != mEnv.end(); ++i) {
        Resolve(i->first, resolving);
    }
}

void
GateGame::Print()
{
    for (auto i = mEnv.begin(); i != mEnv.end(); ++i) {
        assert(i->second->isConstant());
        cout << i->first << " " << static_cast<Constant*>(i->second)->mValue << '\n';
    }
}

int
main()
{
    GateGame g;

    string line;
    while (getline(cin, line)) {
        g.ParseLine(line);
    }

    g.Run();
    /*set<string> resolving;
    cout << g.Resolve("A", resolving) << "\n";*/
    g.Print();
    return 0;
}
