(function(storyContent) {

    var story = new inkjs.Story(storyContent);

		story.delayingContinue = false;
		setupStory(story);
		
    var storyContainer = document.querySelectorAll('#story')[0];

    function showAfter(delay, el) {
        setTimeout(function() { el.classList.add("show"); scrollToBottom(); }, delay);
    }

    function scrollToBottom() {
        var progress = 0.0;
        var start = window.pageYOffset || document.documentElement.scrollTop || document.body.scrollTop || 0;
        var dist = document.body.scrollHeight - window.innerHeight - start;
        if( dist < 0 ) return;

        var duration = 300 + 300*dist/100;
        var startTime = null;
        function step(time) {
            if( startTime == null ) startTime = time;
            var t = (time-startTime) / duration;
            var lerp = 3*t*t - 2*t*t*t;
            window.scrollTo(0, start + lerp*dist);
            if( t < 1 ) requestAnimationFrame(step);
        }
        requestAnimationFrame(step);
    }

    function continueStory() {

        var paragraphIndex = 0;
        var delay = 0.0;
				if (!story.delayingContinue && story.canContinue) {
						// Generate next paragraph of story text
						// Get ink to generate the next paragraph
						var paragraphText = story.Continue();

						// Generate links from URLs:
						var matches = paragraphText.match(/\bhttps?:\/\/\S+/gi);
						
						console.log(matches);
						if (matches !== null && matches.length > 0) {
							matches.forEach(function (match) {
								console.log(match);
								paragraphText = paragraphText.replace(match, '<a href="' + match + '">' + match + '</a>');
								console.log(paragraphText);
							});
						}
				
						// Create paragraph element
						var paragraphElement = document.createElement('p');
						paragraphElement.innerHTML = paragraphText;
						storyContainer.appendChild(paragraphElement);

						// Fade in paragraph after a short delay
						showAfter(delay, paragraphElement);

						delay += 200.0;
				}
				if (!story.delayingContinue && !story.canContinue) {
						// Create HTML choices from ink choices
						story.currentChoices.forEach(function(choice) {

								// Create paragraph with anchor element
								var choiceParagraphElement = document.createElement('p');
								choiceParagraphElement.classList.add("choice");
								choiceParagraphElement.innerHTML = `<a href='#'>${choice.text}</a>`
								storyContainer.appendChild(choiceParagraphElement);

								// Fade choice in after a short delay
								showAfter(delay, choiceParagraphElement);
								delay += 200.0;

								// Click on choice
								var choiceAnchorEl = choiceParagraphElement.querySelectorAll("a")[0];
								choiceAnchorEl.addEventListener("click", function(event) {

										// Don't follow <a> link
										event.preventDefault();

										// Remove all existing choices
										var existingChoices = storyContainer.querySelectorAll('p.choice');
										for(var i=0; i<existingChoices.length; i++) {
												var c = existingChoices[i];
												c.parentNode.removeChild(c);
										}
										// Tell the story where to go next
										story.ChooseChoiceIndex(choice.index);

										continueStory();
								// Aaand loop
								});

								
        scrollToBottom();
            });
				}
				else if (!story.delayingContinue) {
						story.noTimers = false;
						continueStory();
				}
    }
		story.gContinueStory = continueStory;
    continueStory();

})(storyContent);
