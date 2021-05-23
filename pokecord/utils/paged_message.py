import discord
import math

class PagedMessage:
    """
    Given a list of embeds, will send a paged message consisting of page_size embeds per page.
    Reactions can be used to scroll to the first, previous, next, or last page.
    """
    def __init__(self, ctx, bot, embeds, page_size):
        if not embeds or len(embeds) == 0:
            raise ValueError("Embeds must be a non-empty list.")
        if page_size <= 0:
            raise ValueError("page_size must be > 0")

        # the discord context from the bot command
        self.ctx = ctx
        # the discord bot, for waiting for reactions
        self.bot = bot
        # the embeds to show
        self.embeds = embeds
        # the amount of embeds to show per page
        self.page_size = page_size
        # the current visible page number, (0-indexed)
        self.current_page = 0
        # the max page number (0-indexed)
        self.page_count = math.ceil(len(embeds) / page_size) - 1
        # the visible messages shown on the screen. Must be multiple messages
        # because there's a limit of 1 embed per message
        self.visible_messages = [None] * page_size
    
    def _calculate_visible_embeds(self, page_number):
        """
        Given a 0-indexed page number, calculate the visible embeds on it. 
        """
        visible_embeds = []
        starting_index = page_number * self.page_size
        for idx in range(starting_index, starting_index + self.page_size):
            if idx >= len(self.embeds):
                continue
            visible_embeds.append(self.embeds[idx])
        return visible_embeds

    def _check_sender(self, reaction, user):
        """
        Checks that the user matches the author who sent the message that created this PagedMessage.
        """
        return user == self.ctx.author

    async def _change_page(self):
        """
        Update what's visible on the page. Assumes self.current_page has already been updated.
        """
        visible_embeds = self._calculate_visible_embeds(self.current_page)
        for idx, message in enumerate(self.visible_messages):
            if idx < len(visible_embeds):
                # if there is an embed to send, send it
                await message.edit(content="", embed=visible_embeds[idx])
            else:
                await message.edit(embed=discord.Embed(title="No Content"))

    async def send_message(self):
        """
        Sends the initial messages with the first set of embeds.
        """
        self.current_page = 0
        visible_embeds = self._calculate_visible_embeds(self.current_page)
        for idx, embed in enumerate(visible_embeds):
            message = await self.ctx.send(embed=embed)
            self.visible_messages[idx] = message

        # send reactions needed for changing page to the last message in page
        await self.visible_messages[-1].add_reaction('⏪')     
        await self.visible_messages[-1].add_reaction('◀')
        await self.visible_messages[-1].add_reaction('▶')
        await self.visible_messages[-1].add_reaction('⏩')

        print(self.visible_messages)

        reaction = None
        user = None
        while True:
            # process reaction to determine and set next page
            if str(reaction) == '⏪':
                self.current_page = 0
                await self._change_page()

            elif str(reaction) == '◀':
                if self.current_page > 0:
                    self.current_page -= 1
                    await self._change_page()

            if str(reaction) == '▶':
                if self.current_page < self.page_count:
                    self.current_page += 1
                    await self._change_page()

            if str(reaction) == '⏩':
                self.current_page = self.page_count
                await self._change_page()                

            # wait for next reaction to be sent
            try:
                reaction, user = await self.bot.wait_for('reaction_add', check=self._check_sender, timeout=None)
                print(f"Reaction: {reaction}, received from user: {user}")
                await self.visible_messages[-1].remove_reaction(reaction, user)
            except Exception as e:
                print(f"Error when handling paged message, excpetion: {e}")
                break
