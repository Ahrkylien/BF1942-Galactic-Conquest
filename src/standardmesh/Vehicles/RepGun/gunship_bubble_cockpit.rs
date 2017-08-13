subshader "gunship_bubble_cockpit_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.588235;
      	twosided true;
	texture "texture/vehicles/repgun/gunship_bubble_id1";
}

subshader "gunship_bubble_cockpit_Material1" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 1 1 1;
	lightingSpecular false;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap true;
	transparent true;
      	twosided true;
      	depthwrite false;
	texture "texture/vehicles/repgun/gunship_bubble_id2";
}

subshader "gunship_bubble_cockpit_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	texture "texture/vehicles/repgun/gunship_bubble_id3";
      	twosided true;
}

subshader "gunship_bubble_cockpit_Material3" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	texture "texture/vehicles/repgun/gunship_bubble_id4";
      	twosided true;
}

