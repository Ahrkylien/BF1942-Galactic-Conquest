subshader "jud_fakedoor02_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
      materialDiffuse 1.0 0.0862745 0.882353;
	texture "texture/buildings/judicator/jud_floornospec";
}

subshader "jud_fakedoor02_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.243137 0.258824 0.505882;
	texture "texture/buildings/judicator/jud_door01";
}

subshader "jud_fakedoor02_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.796079;
	texture "texture/buildings/judicator/jud_pipe";
}

