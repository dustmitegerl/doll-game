# a narrative game about a doll making decisions (sort of) about how to spend its day. please submit answers to prompts
# verbatim -- same spelling, case sensitive. all paths lead to sleep or being eaten

import random


def unfinished():
    print('sorry, this path is under construction. please choose another. \n')


def wrong():
    print('silly doll! please choose from the given options! letter for letter, dear. case sensitive. \n')


def fight_outcome(choice):
    outcomes = ['1', 'sleepy', 'eaten']
    outcome = random.choice(outcomes)
    if outcome == '1':
        if choice == 'fightBack':
            fight()
        elif choice == 'giveUp':
            give_up()
    elif outcome == 'sleepy':
        sleepy()
    elif outcome == 'eaten':
        eaten()


def fight():
    flip = 0
    flips = random.randrange(0, 13)
    while flip < flips:
        if flip % 2 == 0:
            print('doll is on top!')
        else:
            print('null is on top!')
        flip += 1
    print('obviously, null once again has doll pinned. its eyes reveal an obvious, growing hunger.')
    keep_fighting = input('does doll keep fighting? y/n \n')
    if keep_fighting == 'y':
        fight_outcome('fightBack')
    elif keep_fighting == 'n':
        fight_outcome('giveUp')
    else:
        wrong()
        fight()


def give_up_choice():
    try_again = input('would it like to try again? y/n \n')
    if try_again == 'y':
        fight()
    elif try_again == 'n':
        sleepy()
    else:
        wrong()
        give_up_choice()


def give_up():
    print('probably for the best! nothing disappointing about a doll giving up. \n'
          'null sees the submission in doll\'s eye and rolls off, shifting its lock into a hug \n'
          'should doll try to take advantage, both of them know it would fail. \n'
          'after a moment of rest, null asks doll if it would like to try again. \n')
    give_up_choice()


def end():
    restart = input('the end. wake up? y/n \n')
    if restart == 'y':
        start()
    elif restart == 'n':
        print('thanks for giving the little doll another day of its story. \n')
    else:
        wrong()
        end()


def sleepy():
    print('doll feels sleep suddenly weigh over, somewhat blissfully, somewhat sadly awaiting \n'
          'a rest deeper than any animal\'s somnolence. a final thought, something like a dream \n'
          'slugs through the doll\'s ersatz thoughts like heavy, dusty vapor \n'
          'before it is returned to the Stillness. \n')
    end()


def wrestle_choice():
    struggle = input('does doll struggle? y/n \n')
    if struggle == 'y':
        fight()
    elif struggle == 'n':
        give_up()
    else:
        wrong()
        wrestle_choice()


def wrestle():
    print('though doll is pretty weak, its long limbs and loose joints may serve an advantage \n'
          'against null\'s superior strength \n'
          'well, almost -- but it doesn\'t take long before null\'s got doll caught in her trap! \n')
    wrestle_choice()


def coffee():
    print('ooh, toasty choice! \n'
          'ready to get hyped, doll and null decide to wrestle in the rainy courtyard. \n')
    wrestle()


def fight_choice():
    fightOrNot = input('would doll like to wrestle its friend null? y/n \n')
    if fightOrNot == 'y':
        wrestle()
    elif fightOrNot == 'n':
        print('doll decides not, and after a while, decides to read alone for a while \n'
              'before moving on with its day. \n')
        start()
    else:
        wrong()
        fight_choice()


def lapsang():
    print('oo, smoky choice! \n'
          'doll locks eyes with never, and they nod together, a little gleam matched between them. \n'
          'the coffee must have really cranked null, because suddenly it asks doll if it would like \n'
          'to go wrestle in the parlor. \n')
    fight_choice()


def valerian():
    print('knowing what\'s to come, nix excitedly though already sedatedly lifts dolls hand \n'
          'and guides it to parlor, to the fireplace. each takes one of the couches, angled \n'
          'slightly, side by side. their fingers still ever so slightly clasped, they gaze \n'
          'into the fire, occasionally glancing over to one another, syncing sleepiness \n'
          'with each other\'s eyes. \n')
    sleepy()


def plum():
    print('after cozing with friends awhile, doll decides to get some alone time to read \n'
          'from the house\'s immense library for a while, learning about dragons \n'
          'and artificial biochemistry. after that, it practices drawing, creating pastel \n'
          'pictures of various objects in the common spaces as well as their friends\' rooms. \n')
    sleepy()


def tea_choice():
    tea = input('the house has coffee, lapsang, valerian and plum. \n')
    if tea == 'coffee':
        coffee()
    elif tea == 'lapsang':
        lapsang()
    elif tea == 'valerian':
        valerian()
    elif tea == 'plum':
        plum()
    else:
        wrong()
        tea_choice()


def tea_party():
    print('so the doll wants to have a tea party. how cute :>  \n'
          'so here it is, with a few friends -- null, nix and never, the triplets. \n'
          'null wants coffee (yes, there is coffee); nix wants valerian; never wants lapsang. \n'
          '\nso, what would doll like? \n')
    tea_choice()


def clean():
    print('doll spends all day sweeping, mopping, wiping, scrubbing, shining, kissing and dusting. \n')
    sleepy()


def eaten():
    print('the monster (after all, who does doll know that isn\'t a monster?) pins the weary, broken doll \n'
          'to the floor with little effort. doll feels its nerve-substance leaking from its broken joints \n'
          'and looks pleadingly into the eyes of its opponent, knowing from touch alone what was coming \n'
          'and that there is no way out. giving small struggle, it nonetheless signals submission \n'
          'with a blink and sees it\'s dear friend, overcome with love, close its jaws over the doll\'s face, \n'
          'pulling back to tear it to pieces with claws and sharp teeth. \n'
          'already blinded, the last thing doll senses is the total heat of its loving friend blending with \n'
          'its torn body rupturing, acrid-sweet polyphenols reacting with caustic saliva, sizzling and frothing \n'
          'on its papersoft potash-dusted skin. \n'
          'of course, doll thinks, this isn\'t the end - \n'
          'doll knows her hungry friends will always return it remains, \n'
          'once processed, to their witch so that it can repair it for the next day, or whatever day \n'
          'it gets around to it. \n')
    sleepy()


def start():
    print('so the what does the little doll want to do today?')
    activity = input('it can choose \'tea party\', \'clean the house\', or \'get eaten\'. \n')
    if activity == 'tea party':
        tea_party()
    elif activity == 'clean the house':
        clean()
    elif activity == 'get eaten':
        eaten()
    else:
        start()


start()
